//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1421/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1421<F: Float>(t12161: F, t835: F, t12223: F, t1880: F, t325: F, t723: F, t12225: F, t12232: F, t12252: F, t12256: F, t12263: F, t1445: F, t1998: F, t2194: F, t2201: F, t32944: F, t32946: F, t32951: F, t32953: F, t32955: F, t32958: F, t32960: F, t32963: F, t5694: F, t5703: F, t6159: F, t701: F, t7653: F, t813: F) -> (F, F, F, F, F) {
    let t38961 = t835 * t12161;
    let t38970 = t12223 * t1880;
    let t38974 = t325 * t12161;
    let t38975 = t38974 * t723;
    let t38983 = t32944 + t32946 - t32951 - F::new(0.46011511144704899612e1) * t6159 * t12232 - F::new(0.46011511144704899612e1) * t1998 * t1445 * t38961 * t701 - F::new(0.14300195980740170668e1) * t12256 * t7653 + F::new(0.71500979903700853338e0) * t5703 * t12263 + t32953 + t32955 - t32958 - t32960 + t32963 - F::new(0.46011511144704899612e1) * t2201 * t1445 * t38970 - F::new(0.92023022289409799224e1) * t813 * t1445 * t38975 - F::new(0.92023022289409799224e1) * t2194 * t12225 + F::new(0.92686455430723328401e-1) * t12252 * t5694;
    (t38961, t38970, t38974, t38975, t38983)
}
