//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1864;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta444<F: Float>(t19226: F, t954: F, t11134: F, t11574: F, t15127: F, t15189: F, t15363: F, t15364: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F, t4631: F, t4635: F, t2924: F, t11404: F, t11548: F, t15400: F, t1622: F, t19046: F, t19079: F, t19130: F, t19132: F, t19173: F, t2938: F, t311: F, t4647: F, t4670: F, t6158: F, t6174: F, t6177: F, t946: F, t955: F) -> (F, F, F, F, F) {
        let (t19227, t19247) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1864::<F>(t19226, t954, t11134, t11574, t15127, t15189, t15363, t15364, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19250, t19252, t19253) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1865::<F>(t4631, t4635, t2924, t11404, t11548, t15400, t1622, t19046, t19079, t19130, t19132, t19173, t19227, t19247, t2938, t311, t4647, t4670, t6158, t6174, t6177, t946, t955);
    (t19227, t19247, t19250, t19252, t19253)
}
