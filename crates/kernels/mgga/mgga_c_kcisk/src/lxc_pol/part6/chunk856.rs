//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 856/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk856<F: Float>(t11986: F, t23947: F, t23949: F, t23951: F, t23969: F, t28768: F, t28776: F, t28780: F, t28783: F, t28790: F, t28794: F, t29759: F, t7648: F, t9163: F, t23976: F, t23978: F, t24608: F, t2648: F, t28797: F, t28803: F, t28807: F, t28811: F, t28815: F, t28818: F, t28953: F, t29981: F, t5445: F) -> (F, F) {
    let t30003 = 0.34822083333333333333e-2 * t28768 + 0.46429444444444444443e-2 * t23947 - 0.12381185185185185185e-1 * t23949 - 0.46429444444444444443e-2 * t23951 + 0.27857666666666666666e-1 * t28776 + 0.30952962962962962963e-2 * t28780 + 0.51072388888888888887e-1 * t28783 + 0.579e0 * t7648 * t9163 - 0.43134342e-1 * t11986 * t29759 + 0.69644166666666666665e-2 * t23969 + 0.69644166666666666666e-2 * t28790 + 0.18571777777777777778e-1 * t28794;
    let t30020 = 0.18571777777777777778e-1 * t28797 + 0.18571777777777777778e-1 * t23976 - 0.11607361111111111111e-2 * t28803 - 0.92858888888888888888e-2 * t28807 - 0.15476481481481481482e-1 * t28811 - 0.11607361111111111111e-1 * t28815 - 0.69644166666666666666e-2 * t28818 + 0.46429444444444444443e-2 * t23978 - 0.579e0 * t24608 * t2648 - 0.223494e0 * t5445 * t29759 - 0.17411041666666666666e-2 * t28953 + 0.223494e0 * t5445 * t29981;
    (t30003, t30020)
}
