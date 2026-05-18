//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1150/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1150<F: Float>(t2288: F, t5720: F, t15386: F, t31195: F, t13287: F, t2297: F, t5616: F, t30817: F, t9649: F, t31126: F, t31128: F, t31140: F, t35291: F, t35302: F, t35308: F, t35316: F, t35318: F, t35349: F, t37504: F, t39876: F, t39879: F, t39883: F, t39885: F, t39889: F) -> (F, F) {
    let t39891 = t2288 * t5720;
    let t39893 = t31195 * t15386 * t39891;
    let t39897 = t31195 * t13287 * t2297 * t5616;
    let t39899 = t30817 * t9649;
    let t39904 = t35291 + t35302 - t35308 - F::new(0.18868855373762491241e-2) * t39876 + F::new(0.42874018118069736972e-3) * t39879 - t35316 - t35318 + F::new(0.21437009059034868486e-3) * t39883 + F::new(0.28303283060643736861e-1) * t39885 - F::new(0.25724410870841842183e-2) * t39889 + F::new(0.47172138434406228102e-2) * t39893 - F::new(0.31448092289604152068e-2) * t39897 - F::new(0.12862205435420921092e-2) * t39899 - t35349 + F::new(0.6621875e-1) * t31126 - F::new(0.28582678745379824648e-3) * t31128 + t37504 + F::new(0.7640625e-2) * t31140;
    (t39891, t39904)
}
