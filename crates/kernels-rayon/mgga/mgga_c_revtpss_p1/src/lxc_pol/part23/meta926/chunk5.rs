//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3008/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3008(t11710: f64, t23899: f64, t4892: f64, t1011: f64, t15987: f64, t23503: f64, t19773: f64, t4845: f64, t11875: f64, t15656: f64, t16089: f64, t18946: f64, t19770: f64, t23839: f64, t23911: f64, t23945: f64, t24007: f64, t3091: f64, t3092: f64, t3117: f64, t3162: f64, t3241: f64, t357: f64, t42690: f64, t43238: f64, t43285: f64, t4858: f64, t4866: f64, t4873: f64, t55062: f64, t55065: f64, t55155: f64, t6100: f64, t6271: f64, t6278: f64, t66434: f64, t66702: f64, t67264: f64, t67301: f64, t79410: f64) -> f64 {
    let t79938 = t4892 * t11710 * t23899;
    let t79944 = t1011 * t15987 * t23503;
    let t79946 = t19773 * t4845;
    let t79951 = 0.85748036236139473944e-3_f64 * t16089 * t3092 * t66434 * t4873 + t55062 - t55065 - 0.57165357490759649295e-3_f64 * t67264 - 0.64311027177104605458e-3_f64 * t15656 * t6278 - 0.64311027177104605458e-3_f64 * t4858 * t19770 + 0.42874018118069736972e-3_f64 * t3091 * t3092 * t18946 * t23911 + 0.42874018118069736972e-3_f64 * t3091 * t3092 * t6100 * t79410 + 0.12862205435420921092e-2_f64 * t11875 * t3117 * t6271 * t3162 * t4866 - 0.64311027177104605458e-3_f64 * t42690 * t3117 * t24007 * t66702 * t357 + 0.57165357490759649296e-3_f64 * t79938 + 0.45732285992607719437e-2_f64 * t67301 + t3241 * t23945 / 18.0_f64 - t79944 / 144.0_f64 + t55155 - 0.42874018118069736972e-3_f64 * t79946 - 0.33875767401931644027e-3_f64 * t43238 + 0.12862205435420921092e-2_f64 * t43285 * t23839;
    t79951
}
