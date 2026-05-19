//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1287/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1287<F: Float>(t33195: F, t10667: F, t10721: F, t10996: F, t11075: F, t11096: F, t11113: F, t1445: F, t1890: F, t1966: F, t2004: F, t2033: F, t2049: F, t2194: F, t28443: F, t28450: F, t28454: F, t32186: F, t32504: F, t33179: F, t33183: F, t33187: F, t33194: F, t4673: F, t549: F, t5577: F, t5715: F, t590: F, t813: F) -> F {
    let t33196 = F::cast_from(0.29792074959875355558e-1_f64) * t33195;
    let t33200 = -t28443 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t1445 * t32186 + F::cast_from(0.47667319935800568892e0_f64) * t2004 * t4673 * t10721 - F::cast_from(0.1022478025437886658e1_f64) * t5577 * t11113 - F::cast_from(0.1022478025437886658e1_f64) * t1966 * t1890 * t10667 * t590 + t33179 + t33183 - F::cast_from(0.47667319935800568892e0_f64) * t10996 * t5715 + t33187 - F::cast_from(0.47667319935800568892e0_f64) * t2049 * t11075 - F::cast_from(0.46011511144704899612e1_f64) * t2194 * t11096 + t28450 + t33194 + t28454 + t33196 + F::cast_from(0.79445533226334281486e-1_f64) * t2033 * t549 * t32504;
    t33200
}
