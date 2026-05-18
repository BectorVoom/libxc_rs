//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 849/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk849<F: Float>(t544: F, t8330: F, t1008: F, t1424: F, t1450: F, t1562: F, t1572: F, t1641: F, t2819: F, t2823: F, t2834: F, t2859: F, t2894: F, t4379: F, t4634: F, t541: F, t574: F, t597: F, t6737: F, t6957: F, t6959: F, t6961: F, t6968: F, t6972: F, t6975: F, t6979: F, t8309: F, t8312: F, t8319: F, t8322: F, t8327: F) -> (F, F) {
    let t8331 = t544 * t8330;
    let t8343 = -F::new(0.23005755572352449806e1) * t4634 * t1008 - F::new(0.46011511144704899612e1) * t1641 * t2834 - F::new(0.23005755572352449806e1) * t574 * t8309 + F::new(0.1022478025437886658e1) * t597 * t8312 + F::new(0.47667319935800568892e0) * t2819 * t541 + F::new(0.47667319935800568892e0) * t2823 * t541 + F::new(0.95334639871601137784e0) * t1572 * t8319 - F::new(0.18404604457881959845e2) * t1562 * t8322 - F::new(0.14300195980740170668e1) * t2859 * t6737 - F::new(0.61348681526273199482e1) * t1450 * t8327 - F::new(0.79445533226334281486e-1) * t8331 * t1424 + F::new(0.79445533226334281486e-1) * t4379 * t2894 - F::new(0.11916829983950142223e0) * t6957 + F::new(0.59584149919750711116e-1) * t6959 + F::new(0.29792074959875355558e-1) * t6961 - F::new(0.59584149919750711116e-1) * t6968 - F::new(0.29792074959875355558e-1) * t6972 + F::new(0.38342925953920749676e0) * t6975 - F::new(0.85206502119823888168e-1) * t6979;
    (t8331, t8343)
}
