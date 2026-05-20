//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1193;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta342<F: Float>(t4719: F, t6219: F, t15101: F, t6110: F, t23466: F, t935: F, t2924: F, t19467: F, t4711: F, t981: F, t1699: F, t6400: F, t1079: F, t1695: F, t6244: F, t11133: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23562, t23564, t23565, t23567, t23568, t23570, t23571) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1193::<F>(t4719, t6219, t15101, t6110, t23466, t935, t2924, t19467, t4711, t981, t1699, t6400);
        let (t23583, t23598) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1194::<F>(t1079, t1695, t6244, t11133, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
    (t23562, t23564, t23565, t23567, t23568, t23570, t23571, t23583, t23598)
}
