//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1203/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1203<F: Float>(t1659: F, t7973: F, t1539: F, t309: F, t32181: F, t36433: F, t463: F, t32003: F, t2146: F, t2147: F, t32161: F, t32163: F, t32167: F, t32168: F, t32171: F, t32176: F, t32180: F, t32183: F, t32187: F, t32191: F, t5331: F, t556: F, t609: F, t7877: F) -> F {
    let t36473 = F::cast_from(0.13170898365871023197e1_f64) * t7973 * t1659;
    let t36475 = t1539 * t309;
    let t36477 = t32181 * t36433 * t36475;
    let t36479 = t1539 * t463;
    let t36482 = F::cast_from(0.34694512752820797848e1_f64) * t32003 * t36433 * t36479;
    let t36489 = -F::cast_from(0.17347256376410398924e1_f64) * t32161 + F::cast_from(0.17347256376410398924e1_f64) * t32163 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t2147 * t7877 * t556 - t32167 - F::cast_from(0.17347256376410398924e1_f64) * t32168 + F::cast_from(0.17347256376410398924e1_f64) * t32171 - t32176 + t32180 - t36473 - F::cast_from(0.69389025505641595696e1_f64) * t32183 - F::cast_from(0.34694512752820797848e1_f64) * t36477 + t36482 + F::cast_from(0.34694512752820797848e1_f64) * t32187 + F::cast_from(0.8673628188205199462e0_f64) * t32191 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t2147 * t609 * t5331;
    t36489
}
