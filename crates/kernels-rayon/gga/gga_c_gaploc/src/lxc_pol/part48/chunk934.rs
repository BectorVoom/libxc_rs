//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 934/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk934(t11724: f64, t1445: f64, t2530: f64, t813: f64, t13601: f64, t4614: f64, t13616: f64, t5748: f64, t11936: f64, t3470: f64, t13525: f64, t13538: f64, t13612: f64, t1457: f64, t1890: f64, t1966: f64, t2087: f64, t2103: f64, t3431: f64, t43991: f64, t43994: f64, t44001: f64, t44004: f64, t44009: f64, t44012: f64, t44940: f64, t45826: f64, t45831: f64, t45837: f64, t45842: f64, t45848: f64, t590: f64, t723: f64, t833: f64, t8483: f64) -> f64 {
    let t45856 = 0.92023022289409799224e1_f64 * t813 * t1445 * t11724 * t2530;
    let t45863 = 0.12269736305254639897e2_f64 * t813 * t4614 * t13601;
    let t45869 = 0.36809208915763919689e2_f64 * t5748 * t4614 * t13616;
    let t45874 = 0.25025342966295298669e1_f64 * t11936 * t3470;
    let t45875 = -0.95857314884801874192e0_f64 * t45826 - t45831 - 0.51123901271894332902e0_f64 * t1966 * t1890 * t13525 * t590 + t45837 - 0.13803453343411469884e2_f64 * t2087 * t1445 * t8483 * t3431 - 0.69017266717057349418e1_f64 * t2087 * t1445 * t45842 * t723 + t45848 + 0.30674340763136599741e2_f64 * t833 * t4614 * t13538 + 0.59584149919750711116e-1_f64 * t43991 - t45856 - t43994 + 0.63904876589867916128e-1_f64 * t44001 + 0.38342925953920749677e1_f64 * t44004 + 0.63904876589867916128e-1_f64 * t44009 + 0.11916829983950142223e0_f64 * t44012 - t45863 - 0.18404604457881959845e2_f64 * t2087 * t4614 * t13612 + t45869 + 0.71500979903700853338e0_f64 * t2103 * t1457 * t44940 - t45874;
    t45875
}
