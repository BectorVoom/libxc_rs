//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 439/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk439<F: Float>(t633: F, t1790: F, t1719: F, t183: F, t164: F, t167: F, t1717: F, t1721: F, t1753: F, t1783: F, t588: F, t600: F, t621: F) -> (F, F, F) {
    let t1791 = t633 * t633;
    let t1792 = t1790 * t1791;
    let t1795 = t183 * t1719;
    let t1812 = 0.13170898365871023197e1 * t1717 * t1795 * t1721 - 0.13170898365871023197e1 * t588 * t621 * t600 * t164 - 0.65854491829355115987e0 * t588 * t183 * t1753 * t164 - 0.65854491829355115987e0 * t588 * t1795 * t164 + 0.65854491829355115987e0 * t167 * t1783;
    (t1791, t1792, t1812)
}
