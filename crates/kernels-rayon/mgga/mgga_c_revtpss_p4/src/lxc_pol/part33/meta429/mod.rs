//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1535;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1536;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1537;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1538;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta429(t19226: f64, t954: f64, t11134: f64, t11574: f64, t15127: f64, t15189: f64, t15363: f64, t15364: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64, t4631: f64, t4635: f64, t2924: f64, t11404: f64, t11548: f64, t15400: f64, t1622: f64, t19046: f64, t19079: f64, t19130: f64, t19132: f64, t19173: f64, t2938: f64, t311: f64, t4647: f64, t4670: f64, t6158: f64, t6174: f64, t6177: f64, t946: f64, t955: f64, t11387: f64, t6109: f64, t934: f64, t11385: f64, t953: f64, t4669: f64, t2970: f64, t6173: f64, t4673: f64, t11452: f64, t6157: f64, t6190: f64, t972: f64, t11409: f64, t11450: f64, t15104: f64, t15350: f64, t15406: f64, t15413: f64, t2943: f64, t2968: f64, t3012: f64, t4652: f64, t4674: f64, t4690: f64, t4712: f64, t1634: f64, t4707: f64, t6209: f64, t6206: f64, t3014: f64, t6205: f64, t4711: f64, t11509: f64, t6189: f64, t15101: f64, t4595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19227, t19247) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1535(t19226, t954, t11134, t11574, t15127, t15189, t15363, t15364, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19252, t19253) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1536(t4631, t4635, t2924, t11404, t11548, t15400, t1622, t19046, t19079, t19130, t19132, t19173, t19227, t19247, t2938, t311, t4647, t4670, t6158, t6174, t6177, t946, t955);
        let (t19258, t19263, t19266, t19269, t19272, t19275) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1537(t11387, t6109, t934, t11385, t6158, t953, t1622, t4669, t6177, t6174, t2970, t6173);
        let t19293 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1538(t19275, t953, t4669, t4673, t11452, t6157, t6190, t972, t11409, t11450, t15104, t15350, t15406, t15413, t19258, t19263, t19266, t19269, t19272, t2943, t2968, t3012, t4652, t4674, t4690, t4712);
        let (t19294, t19297, t19300, t19304, t19307, t19311, t19315) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1539(t1634, t4707, t6209, t972, t6206, t3014, t6205, t4711, t11509, t6189, t15101, t4595);
    (t19252, t19253, t19258, t19293, t19294, t19297, t19300, t19304, t19307, t19311, t19315)
}
