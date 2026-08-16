//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3803/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3803(t12587: f64, t6748: f64, t1300: f64, t198: f64, t336: f64, t3798: f64, t44126: f64, t5023: f64, t6752: f64, t68631: f64, t68633: f64, t68636: f64, t68640: f64, t68673: f64, t68683: f64, t68686: f64, t68689: f64, t68692: f64, t68694: f64, t68696: f64, t68698: f64, t72797: f64, t72832: f64, t72865: f64, t72899: f64, t72925: f64, t72956: f64, t72986: f64, t73020: f64, t73049: f64, t73082: f64, t73109: f64, t73146: f64, t73177: f64, t73210: f64, t73244: f64) -> f64 {
    let t73252 = t6748 * t12587;
    let t73260 = t68631 + t68633 + t68636 + t68640 + t198 * t336 * (t68673 + t72797 + t72832 + t72865 + t72899 + t72925 + t72956 + t72986 + t73020 + t73049 + t73082 + t73109 + t73146 + t73177 + t73210 + t73244) * t1300 - t68683 - t68686 - t68689 - t68692 - t68694 + t68696 - t68698 + 2.0_f64 * t5023 * t73252 * t3798 - 6.0_f64 * t5023 * t6752 * t44126 * t3798;
    t73260
}
