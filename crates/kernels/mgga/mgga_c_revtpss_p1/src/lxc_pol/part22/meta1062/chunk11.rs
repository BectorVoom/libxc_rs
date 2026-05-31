//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3803/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3803<F: Float>(t12587: F, t6748: F, t1300: F, t198: F, t336: F, t3798: F, t44126: F, t5023: F, t6752: F, t68631: F, t68633: F, t68636: F, t68640: F, t68673: F, t68683: F, t68686: F, t68689: F, t68692: F, t68694: F, t68696: F, t68698: F, t72797: F, t72832: F, t72865: F, t72899: F, t72925: F, t72956: F, t72986: F, t73020: F, t73049: F, t73082: F, t73109: F, t73146: F, t73177: F, t73210: F, t73244: F) -> F {
    let t73252 = t6748 * t12587;
    let t73260 = t68631 + t68633 + t68636 + t68640 + t198 * t336 * (t68673 + t72797 + t72832 + t72865 + t72899 + t72925 + t72956 + t72986 + t73020 + t73049 + t73082 + t73109 + t73146 + t73177 + t73210 + t73244) * t1300 - t68683 - t68686 - t68689 - t68692 - t68694 + t68696 - t68698 + F::cast_from(2.0_f64) * t5023 * t73252 * t3798 - F::cast_from(6.0_f64) * t5023 * t6752 * t44126 * t3798;
    t73260
}
