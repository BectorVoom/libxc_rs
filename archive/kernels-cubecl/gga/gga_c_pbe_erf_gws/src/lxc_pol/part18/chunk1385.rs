//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1385/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1385<F: Float>(t1167: F, t12275: F, t50832: F, t3928: F, t810: F, t15118: F, t2429: F, t1172: F, t13751: F, t320: F, t3717: F, t3931: F, t3946: F, t4062: F, t4063: F, t50839: F, t52801: F, t52810: F, t52812: F, t52821: F, t52836: F, t56057: F, t56060: F, t56109: F, t56155: F, t56189: F, t56223: F, t56259: F, t56294: F, t56330: F, t56381: F, t56425: F, t56480: F, t56518: F, t56563: F, t56601: F, t56653: F, t56694: F, t56735: F, t56775: F, t56823: F, t57264: F, t57298: F, t57347: F, t57381: F, t57401: F, t57445: F, t57490: F, t57536: F, t57580: F, t57618: F, t57656: F, t57691: F, t57737: F, t57767: F, t945: F) -> F {
    let t57779 = t12275 * t1167;
    let t57780 = t50832 * t57779;
    let t57785 = t3928 * t810;
    let t57789 = t2429 * t15118;
    let t57791 = -F::cast_from(2.0_f64) * t56057 + t52801 + t56060 - t52810 - t52812 + t1172 * t320 * (t56294 + t56189 + t57381 + t56381 + t57298 + t56775 + t57401 + t56109 + t56694 + t56425 + t57656 + t57445 + t57767 + t57347 + t57580 + t56735 + t56653 + t57264 + t56518 + t56480 + t57490 + t56155 + t57618 + t56823 + t56330 + t56601 + t57737 + t56563 + t57536 + t56223 + t56259 + t57691) * t945 - t52821 + F::cast_from(2.0_f64) * t4062 * t50839 * t3931 - F::cast_from(6.0_f64) * t57780 + F::cast_from(3.0_f64) * t3946 * t13751 * t3717 + t52836 - F::cast_from(3.0_f64) * t3946 * t4063 * t57785 + F::cast_from(6.0_f64) * t57789;
    t57791
}
