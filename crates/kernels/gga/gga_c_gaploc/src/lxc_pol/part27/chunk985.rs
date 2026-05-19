//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 985/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk985<F: Float>(t10972: F, t1445: F, t813: F, t3477: F, t5771: F, t10713: F, t1457: F, t2103: F, t10717: F, t3470: F, t8478: F, t8638: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10973 = t1445 * t10972;
    let t10975 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t10973;
    let t10977 = F::cast_from(0.71500979903700853338e0_f64) * t5771 * t3477;
    let t10978 = t1457 * t10713;
    let t10980 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t10978;
    let t10981 = t1457 * t10717;
    let t10983 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t10981;
    let t10988 = F::cast_from(0.10725146985555128001e1_f64) * t8478 * t3470;
    let t10990 = F::cast_from(0.10725146985555128001e1_f64) * t8638 * t3470;
    (t10973, t10975, t10977, t10978, t10980, t10981, t10983, t10988, t10990)
}
