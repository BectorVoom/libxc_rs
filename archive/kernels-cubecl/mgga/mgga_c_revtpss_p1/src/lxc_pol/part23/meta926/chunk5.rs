//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3008/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3008<F: Float>(t11710: F, t23899: F, t4892: F, t1011: F, t15987: F, t23503: F, t19773: F, t4845: F, t11875: F, t15656: F, t16089: F, t18946: F, t19770: F, t23839: F, t23911: F, t23945: F, t24007: F, t3091: F, t3092: F, t3117: F, t3162: F, t3241: F, t357: F, t42690: F, t43238: F, t43285: F, t4858: F, t4866: F, t4873: F, t55062: F, t55065: F, t55155: F, t6100: F, t6271: F, t6278: F, t66434: F, t66702: F, t67264: F, t67301: F, t79410: F) -> F {
    let t79938 = t4892 * t11710 * t23899;
    let t79944 = t1011 * t15987 * t23503;
    let t79946 = t19773 * t4845;
    let t79951 = F::cast_from(0.85748036236139473944e-3_f64) * t16089 * t3092 * t66434 * t4873 + t55062 - t55065 - F::cast_from(0.57165357490759649295e-3_f64) * t67264 - F::cast_from(0.64311027177104605458e-3_f64) * t15656 * t6278 - F::cast_from(0.64311027177104605458e-3_f64) * t4858 * t19770 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t18946 * t23911 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t6100 * t79410 + F::cast_from(0.12862205435420921092e-2_f64) * t11875 * t3117 * t6271 * t3162 * t4866 - F::cast_from(0.64311027177104605458e-3_f64) * t42690 * t3117 * t24007 * t66702 * t357 + F::cast_from(0.57165357490759649296e-3_f64) * t79938 + F::cast_from(0.45732285992607719437e-2_f64) * t67301 + t3241 * t23945 / F::cast_from(18.0_f64) - t79944 / F::cast_from(144.0_f64) + t55155 - F::cast_from(0.42874018118069736972e-3_f64) * t79946 - F::cast_from(0.33875767401931644027e-3_f64) * t43238 + F::cast_from(0.12862205435420921092e-2_f64) * t43285 * t23839;
    t79951
}
