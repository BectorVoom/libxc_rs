//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 591/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk591<F: Float>(t3219: F, t3222: F, t3228: F, t3232: F, t3236: F, t3241: F, t3245: F, t3248: F, t3251: F, t3256: F, t3260: F, t3182: F, t3184: F, t3190: F, t3194: F, t3199: F, t3202: F, t3204: F, t3207: F, t3210: F, t3213: F) -> F {
    let t3562 = -F::cast_from(0.82073827867876094584e-5_f64) * t3219 - F::cast_from(0.27357942622625364861e-5_f64) * t3222 + F::cast_from(0.39896999657995323756e-6_f64) * t3228 - F::cast_from(0.82073827867876094584e-5_f64) * t3232 - F::cast_from(0.11742981196020707897e-4_f64) * t3236 + F::cast_from(0.11742981196020707897e-4_f64) * t3241 + F::cast_from(0.11742981196020707897e-5_f64) * t3245 - F::cast_from(0.20879020566524818641e-5_f64) * t3248 - F::cast_from(0.11742981196020707897e-4_f64) * t3251 - F::cast_from(0.342503618217270647e-5_f64) * t3256 + F::cast_from(0.11742981196020707897e-4_f64) * t3260;
    let t3563 = -F::cast_from(0.93943849568165663177e-3_f64) * t3182 + F::cast_from(0.56366309740899397906e-3_f64) * t3184 - F::cast_from(0.56366309740899397906e-3_f64) * t3190 - F::cast_from(0.18788769913633132635e-4_f64) * t3194 + F::cast_from(0.33406432906439709826e-4_f64) * t3199 + F::cast_from(0.56366309740899397906e-3_f64) * t3202 + F::cast_from(0.74372214241464483348e-4_f64) * t3204 - F::cast_from(0.56366309740899397906e-3_f64) * t3207 - F::cast_from(0.3556532540941297432e-4_f64) * t3210 + F::cast_from(0.82073827867876094584e-5_f64) * t3213 + t3562;
    t3563
}
