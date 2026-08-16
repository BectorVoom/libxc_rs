//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1150/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1150(t2288: f64, t5720: f64, t15386: f64, t31195: f64, t13287: f64, t2297: f64, t5616: f64, t30817: f64, t9649: f64, t31126: f64, t31128: f64, t31140: f64, t35291: f64, t35302: f64, t35308: f64, t35316: f64, t35318: f64, t35349: f64, t37504: f64, t39876: f64, t39879: f64, t39883: f64, t39885: f64, t39889: f64) -> (f64, f64) {
    let t39891 = t2288 * t5720;
    let t39893 = t31195 * t15386 * t39891;
    let t39897 = t31195 * t13287 * t2297 * t5616;
    let t39899 = t30817 * t9649;
    let t39904 = t35291 + t35302 - t35308 - 0.18868855373762491241e-2_f64 * t39876 + 0.42874018118069736972e-3_f64 * t39879 - t35316 - t35318 + 0.21437009059034868486e-3_f64 * t39883 + 0.28303283060643736861e-1_f64 * t39885 - 0.25724410870841842183e-2_f64 * t39889 + 0.47172138434406228102e-2_f64 * t39893 - 0.31448092289604152068e-2_f64 * t39897 - 0.12862205435420921092e-2_f64 * t39899 - t35349 + 0.6621875e-1_f64 * t31126 - 0.28582678745379824648e-3_f64 * t31128 + t37504 + 0.7640625e-2_f64 * t31140;
    (t39891, t39904)
}
