//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1385/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1385(t1167: f64, t12275: f64, t50832: f64, t3928: f64, t810: f64, t15118: f64, t2429: f64, t1172: f64, t13751: f64, t320: f64, t3717: f64, t3931: f64, t3946: f64, t4062: f64, t4063: f64, t50839: f64, t52801: f64, t52810: f64, t52812: f64, t52821: f64, t52836: f64, t56057: f64, t56060: f64, t56109: f64, t56155: f64, t56189: f64, t56223: f64, t56259: f64, t56294: f64, t56330: f64, t56381: f64, t56425: f64, t56480: f64, t56518: f64, t56563: f64, t56601: f64, t56653: f64, t56694: f64, t56735: f64, t56775: f64, t56823: f64, t57264: f64, t57298: f64, t57347: f64, t57381: f64, t57401: f64, t57445: f64, t57490: f64, t57536: f64, t57580: f64, t57618: f64, t57656: f64, t57691: f64, t57737: f64, t57767: f64, t945: f64) -> f64 {
    let t57779 = t12275 * t1167;
    let t57780 = t50832 * t57779;
    let t57785 = t3928 * t810;
    let t57789 = t2429 * t15118;
    let t57791 = -2.0_f64 * t56057 + t52801 + t56060 - t52810 - t52812 + t1172 * t320 * (t56294 + t56189 + t57381 + t56381 + t57298 + t56775 + t57401 + t56109 + t56694 + t56425 + t57656 + t57445 + t57767 + t57347 + t57580 + t56735 + t56653 + t57264 + t56518 + t56480 + t57490 + t56155 + t57618 + t56823 + t56330 + t56601 + t57737 + t56563 + t57536 + t56223 + t56259 + t57691) * t945 - t52821 + 2.0_f64 * t4062 * t50839 * t3931 - 6.0_f64 * t57780 + 3.0_f64 * t3946 * t13751 * t3717 + t52836 - 3.0_f64 * t3946 * t4063 * t57785 + 6.0_f64 * t57789;
    t57791
}
