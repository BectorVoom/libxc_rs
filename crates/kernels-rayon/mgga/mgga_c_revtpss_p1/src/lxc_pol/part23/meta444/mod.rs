//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1864;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta444(t19226: f64, t954: f64, t11134: f64, t11574: f64, t15127: f64, t15189: f64, t15363: f64, t15364: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64, t4631: f64, t4635: f64, t2924: f64, t11404: f64, t11548: f64, t15400: f64, t1622: f64, t19046: f64, t19079: f64, t19130: f64, t19132: f64, t19173: f64, t2938: f64, t311: f64, t4647: f64, t4670: f64, t6158: f64, t6174: f64, t6177: f64, t946: f64, t955: f64) -> (f64, f64, f64, f64, f64) {
        let (t19227, t19247) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1864(t19226, t954, t11134, t11574, t15127, t15189, t15363, t15364, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19250, t19252, t19253) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1865(t4631, t4635, t2924, t11404, t11548, t15400, t1622, t19046, t19079, t19130, t19132, t19173, t19227, t19247, t2938, t311, t4647, t4670, t6158, t6174, t6177, t946, t955);
    (t19227, t19247, t19250, t19252, t19253)
}
