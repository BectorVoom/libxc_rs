//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1290/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1290(t33195: f64, t10667: f64, t10721: f64, t10996: f64, t11075: f64, t11096: f64, t11113: f64, t1445: f64, t1890: f64, t1966: f64, t2004: f64, t2033: f64, t2049: f64, t2194: f64, t28443: f64, t28450: f64, t28454: f64, t32186: f64, t32504: f64, t33179: f64, t33183: f64, t33187: f64, t33194: f64, t4673: f64, t549: f64, t5577: f64, t5715: f64, t590: f64, t813: f64) -> f64 {
    let t33196 = 0.29792074959875355558e-1_f64 * t33195;
    let t33200 = -t28443 - 0.92023022289409799224e1_f64 * t813 * t1445 * t32186 + 0.47667319935800568892e0_f64 * t2004 * t4673 * t10721 - 0.1022478025437886658e1_f64 * t5577 * t11113 - 0.1022478025437886658e1_f64 * t1966 * t1890 * t10667 * t590 + t33179 + t33183 - 0.47667319935800568892e0_f64 * t10996 * t5715 + t33187 - 0.47667319935800568892e0_f64 * t2049 * t11075 - 0.46011511144704899612e1_f64 * t2194 * t11096 + t28450 + t33194 + t28454 + t33196 + 0.79445533226334281486e-1_f64 * t2033 * t549 * t32504;
    t33200
}
