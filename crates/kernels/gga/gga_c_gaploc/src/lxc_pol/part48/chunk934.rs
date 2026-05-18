//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 934/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk934<F: Float>(t11724: F, t1445: F, t2530: F, t813: F, t13601: F, t4614: F, t13616: F, t5748: F, t11936: F, t3470: F, t13525: F, t13538: F, t13612: F, t1457: F, t1890: F, t1966: F, t2087: F, t2103: F, t3431: F, t43991: F, t43994: F, t44001: F, t44004: F, t44009: F, t44012: F, t44940: F, t45826: F, t45831: F, t45837: F, t45842: F, t45848: F, t590: F, t723: F, t833: F, t8483: F) -> F {
    let t45856 = F::new(0.92023022289409799224e1) * t813 * t1445 * t11724 * t2530;
    let t45863 = F::new(0.12269736305254639897e2) * t813 * t4614 * t13601;
    let t45869 = F::new(0.36809208915763919689e2) * t5748 * t4614 * t13616;
    let t45874 = F::new(0.25025342966295298669e1) * t11936 * t3470;
    let t45875 = -F::new(0.95857314884801874192e0) * t45826 - t45831 - F::new(0.51123901271894332902e0) * t1966 * t1890 * t13525 * t590 + t45837 - F::new(0.13803453343411469884e2) * t2087 * t1445 * t8483 * t3431 - F::new(0.69017266717057349418e1) * t2087 * t1445 * t45842 * t723 + t45848 + F::new(0.30674340763136599741e2) * t833 * t4614 * t13538 + F::new(0.59584149919750711116e-1) * t43991 - t45856 - t43994 + F::new(0.63904876589867916128e-1) * t44001 + F::new(0.38342925953920749677e1) * t44004 + F::new(0.63904876589867916128e-1) * t44009 + F::new(0.11916829983950142223e0) * t44012 - t45863 - F::new(0.18404604457881959845e2) * t2087 * t4614 * t13612 + t45869 + F::new(0.71500979903700853338e0) * t2103 * t1457 * t44940 - t45874;
    t45875
}
