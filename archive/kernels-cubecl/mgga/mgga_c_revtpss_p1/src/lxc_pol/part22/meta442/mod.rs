//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2084;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta442<F: Float>(t1568: F, t2718: F, t4469: F, t822: F, t10923: F, t10925: F, t10930: F, t10935: F, t10939: F, t10948: F, t10961: F, t10964: F, t10966: F, t10969: F, t10971: F, t10974: F, t14507: F, t2646: F, t2724: F, t4514: F, t4526: F, t820: F, t837: F, t14540: F, t14572: F, t14953: F, t868: F, t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14961, t14972, t14976) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2084::<F>(t1568, t2718, t4469, t822, t10923, t10925, t10930, t10935, t10939, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t14507, t2646, t2724, t4514, t4526, t820, t837);
        let (t14978, t14979, t14982, t14983, t14985, t14986, t14987) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2085::<F>(t14540, t14572, t14953, t14976, t868, t4533, t72, t686, t2465, t1569, t867, t786);
    (t14961, t14972, t14978, t14979, t14982, t14983, t14985, t14986, t14987)
}
