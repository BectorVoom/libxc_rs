//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 834/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk834<F: Float>(t213: F, t683: F, t171: F, t6793: F, t1113: F, t1420: F, t17807: F, t27521: F, t30671: F, t30779: F, t33388: F, t33394: F, t33434: F, t33436: F, t33445: F, t35395: F, t35402: F, t35406: F, t35410: F, t35416: F, t35420: F, t35426: F, t35427: F, t35431: F, t35435: F, t52: F, t6758: F, t7456: F, t7457: F, t7470: F) -> (F, F, F) {
    let t35437 = t683 * t213;
    let t35438 = t6793 * t171 * t35437;
    let t35441 = -F::new(0.44455354858818847408e-2) * t7456 * t52 * t7457 * t1113 + F::new(0.22227677429409423704e-2) * t30671 * t35395 - F::new(0.22227677429409423704e-2) * t33388 * t35395 + F::new(0.52700762016626893448e-4) * t7456 * t35402 + F::new(0.39129660776942540761e-2) * t33445 * t35406 + F::new(0.68116566383613497688e-3) * t30779 * t7470 * t35410 - F::new(0.68116566383613497688e-3) * t27521 * t35416 - F::new(0.76612330055555555556e-1) * t35420 * t1420 - F::new(0.22979081259345929704e-6) * t17807 * t33394 * t6758 + F::new(0.11738898233082762228e-1) * t35426 * t33436 * t35427 - F::new(0.17608347349624143343e-1) * t33434 * t33436 * t35431 + F::new(0.42300125954037691564e-4) * t35435 * t35438;
    (t35437, t35438, t35441)
}
