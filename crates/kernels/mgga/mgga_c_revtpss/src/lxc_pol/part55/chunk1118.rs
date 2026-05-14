//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1118/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1118<F: Float>(t102019: F, t1937: F, t111018: F, t28653: F, t6993: F, t2014: F, t32734: F, t5542: F, t4292: F, t651: F, t8686: F, t32385: F, t4248: F, t27123: F, t8641: F, t27126: F) -> (F, F, F, F, F, F, F, F) {
    let t128977 = 2.0 * t102019 * t1937;
    let t128979 = 2.0 * t111018 * t1937;
    let t128981 = 2.0 * t28653 * t6993;
    let t128983 = t2014 * t32734 * t5542;
    let t128986 = t651 * t8686 * t4292;
    let t128988 = t4248 * t32385;
    let t128990 = t27123 * t8641;
    let t128992 = t27126 * t8641;
    (t128977, t128979, t128981, t128983, t128986, t128988, t128990, t128992)
}
