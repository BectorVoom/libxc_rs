//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 896/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk896<F: Float>(t448: F, t8396: F, t315: F, t7966: F, t2137: F, t7943: F, t33428: F, t2134: F, t32066: F, t32073: F, t32080: F, t32082: F, t32084: F, t33778: F, t33783: F, t33786: F, t33789: F, t33794: F, t7912: F, t7935: F, t9015: F) -> (F,) {
    let t33795 = t8396 * t448;
    let t33796 = t315 * t33795;
    let t33798 = 0.17347256376410398924e1 * t33796 * t7966;
    let t33799 = t2137 * t33795;
    let t33801 = 0.17347256376410398924e1 * t33799 * t7943;
    let t33802 = t315 * t33428;
    let t33804 = 0.17347256376410398924e1 * t33802 * t2134;
    let t33810 = -0.17347256376410398924e1 * t33778 * t7935 + 0.65854491829355115987e0 * t32066 + t33783 - t32073 - t33786 + 0.8673628188205199462e0 * t33789 - t33794 + t33798 - t33801 - t33804 + 0.13170898365871023197e1 * t32080 - 0.26341796731742046394e1 * t32082 - 0.13170898365871023197e1 * t32084 + 0.8673628188205199462e0 * t7912 * t9015;
    (t33810,)
}
