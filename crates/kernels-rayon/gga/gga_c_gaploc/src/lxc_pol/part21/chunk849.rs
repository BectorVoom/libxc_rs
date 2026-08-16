//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 849/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk849(t544: f64, t8330: f64, t1008: f64, t1424: f64, t1450: f64, t1562: f64, t1572: f64, t1641: f64, t2819: f64, t2823: f64, t2834: f64, t2859: f64, t2894: f64, t4379: f64, t4634: f64, t541: f64, t574: f64, t597: f64, t6737: f64, t6957: f64, t6959: f64, t6961: f64, t6968: f64, t6972: f64, t6975: f64, t6979: f64, t8309: f64, t8312: f64, t8319: f64, t8322: f64, t8327: f64) -> (f64, f64) {
    let t8331 = t544 * t8330;
    let t8343 = -0.23005755572352449806e1_f64 * t4634 * t1008 - 0.46011511144704899612e1_f64 * t1641 * t2834 - 0.23005755572352449806e1_f64 * t574 * t8309 + 0.1022478025437886658e1_f64 * t597 * t8312 + 0.47667319935800568892e0_f64 * t2819 * t541 + 0.47667319935800568892e0_f64 * t2823 * t541 + 0.95334639871601137784e0_f64 * t1572 * t8319 - 0.18404604457881959845e2_f64 * t1562 * t8322 - 0.14300195980740170668e1_f64 * t2859 * t6737 - 0.61348681526273199482e1_f64 * t1450 * t8327 - 0.79445533226334281486e-1_f64 * t8331 * t1424 + 0.79445533226334281486e-1_f64 * t4379 * t2894 - 0.11916829983950142223e0_f64 * t6957 + 0.59584149919750711116e-1_f64 * t6959 + 0.29792074959875355558e-1_f64 * t6961 - 0.59584149919750711116e-1_f64 * t6968 - 0.29792074959875355558e-1_f64 * t6972 + 0.38342925953920749676e0_f64 * t6975 - 0.85206502119823888168e-1_f64 * t6979;
    (t8331, t8343)
}
