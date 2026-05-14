//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1216/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1216<F: Float>(t1339: F, t33544: F, t5635: F, t9461: F, t3759: F, t6221: F, t9438: F, t2718: F, t32019: F, t33389: F, t33521: F, t33524: F, t33527: F, t33530: F, t33533: F, t33535: F, t33542: F, t9426: F, t9796: F) -> (F, F, F, F, F) {
    let t33545 = t1339 * t33544;
    let t33547 = t9461 * t5635;
    let t33548 = t3759 * t33547;
    let t33550 = t6221 * t9438;
    let t33553 = -0.10416666666666666667e-1 * t33521 * t2718 - 0.10416666666666666667e-1 * t33524 * t2718 - 0.10416666666666666667e-1 * t33527 * t2718 + 0.11054629629629629629e-2 * t33530 - 0.44218518518518518517e-2 * t33533 - 0.16581944444444444444e-2 * t33535 - 0.120625e-1 * t9426 * t33389 + 0.10416666666666666667e-1 * t32019 * t9796 + 0.11054629629629629629e-2 * t33542 - 0.33163888888888888888e-2 * t33545 + 0.27636574074074074073e-2 * t33548 + 0.27777777777777777779e-1 * t33550 * t2718;
    (t33545, t33547, t33548, t33550, t33553)
}
