//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 801/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk801<F: Float>(t11095: F, t22513: F, t22518: F, t22522: F, t22524: F, t22528: F, t22534: F, t22536: F, t22541: F, t22543: F, t22548: F, t22549: F, t22552: F, t22553: F, t22557: F, t22559: F, t22565: F, t22568: F, t22574: F, t22577: F, t22583: F, t22587: F, t22590: F, t22591: F, t22593: F, t22597: F, t22598: F, t22603: F, t22606: F, t22610: F, t22613: F, t5540: F, t5569: F, t5570: F, t5572: F, t5579: F, t5591: F, t5611: F, t73: F) -> (F,) {
    let t22617 = -0.30274029503828221194e-3 * t22513 * t22518 + 0.25537443351851851852e-1 * t22522 * t5570 * t22524 + 0.22270151833971792333e-3 * t5569 * t5570 * t22528 - 0.14836531933660919214e-4 * t22534 * t5570 * t22536 - 0.25537443351851851852e-1 * t22541 * t5570 * t22543 - 0.51789017496114396277e-5 * t22548 * t22549 + 0.15322466011111111111e0 * t22552 * t5579 * t22553 - 0.18164417702296932716e-2 * t22557 * t5591 * t22559 - 0.27568129967481981592e-3 * t22565 * t11095 - 0.11877414311451622578e-2 * t5569 * t22568 * t5572 + 0.14846767889314528222e-3 * t22574 - 0.44540303667943584666e-4 * t5569 * t5570 * t22577 + 0.14846767889314528222e-3 * t22583 * t22587 + 0.88910709717637694816e-2 * t22590 * t22591 * t22593 + 0.10338048737805743098e-3 * t22597 * t5540 * t22598 - 0.51690243689028715488e-4 * t22603 * t22606 + 0.12768721675925925926e-1 * t5611 * t22610 + 0.89080607335887169332e-3 * t22613 * t73 * t22598;
    (t22617,)
}
