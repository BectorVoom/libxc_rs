//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1528;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1529;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1530;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1531;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta426<F: Float>(t19226: F, t954: F, t11134: F, t11574: F, t15127: F, t15189: F, t15363: F, t15364: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F, t4631: F, t4635: F, t2924: F, t11404: F, t11548: F, t15400: F, t1622: F, t19046: F, t19079: F, t19130: F, t19132: F, t19173: F, t2938: F, t311: F, t4647: F, t4670: F, t6158: F, t6174: F, t6177: F, t946: F, t955: F, t11387: F, t6109: F, t934: F, t11385: F, t953: F, t4669: F, t2970: F, t6173: F, t4673: F, t11452: F, t6157: F, t6190: F, t972: F, t11409: F, t11450: F, t15104: F, t15350: F, t15406: F, t15413: F, t2943: F, t2968: F, t3012: F, t4652: F, t4674: F, t4690: F, t4712: F, t1634: F, t4707: F, t6209: F, t6206: F, t3014: F, t6205: F, t4711: F, t11509: F, t6189: F, t15101: F, t4595: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19227, t19247) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1528::<F>(t19226, t954, t11134, t11574, t15127, t15189, t15363, t15364, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19252, t19253) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1529::<F>(t4631, t4635, t2924, t11404, t11548, t15400, t1622, t19046, t19079, t19130, t19132, t19173, t19227, t19247, t2938, t311, t4647, t4670, t6158, t6174, t6177, t946, t955);
        let (t19258, t19263, t19266, t19269, t19272, t19275) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1530::<F>(t11387, t6109, t934, t11385, t6158, t953, t1622, t4669, t6177, t6174, t2970, t6173);
        let t19293 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1531::<F>(t19275, t953, t4669, t4673, t11452, t6157, t6190, t972, t11409, t11450, t15104, t15350, t15406, t15413, t19258, t19263, t19266, t19269, t19272, t2943, t2968, t3012, t4652, t4674, t4690, t4712);
        let (t19294, t19297, t19300, t19304, t19307, t19311, t19315) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1532::<F>(t1634, t4707, t6209, t972, t6206, t3014, t6205, t4711, t11509, t6189, t15101, t4595);
    (t19252, t19253, t19258, t19293, t19294, t19297, t19300, t19304, t19307, t19311, t19315)
}
