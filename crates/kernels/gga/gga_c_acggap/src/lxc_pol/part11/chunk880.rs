//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 880/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk880<F: Float>(t1603: F, t618: F, t2137: F, t2140: F, t1614: F, t7976: F, t29988: F, t557: F, t1265: F, t2122: F, t2146: F, t29973: F, t29977: F, t29982: F, t29986: F, t29989: F, t33414: F, t33416: F, t463: F, t7912: F, t8004: F, t8392: F, t8400: F, t8411: F, t8791: F, t9010: F, t939: F) -> (F, F) {
    let t33428 = t1603 * t618;
    let t33429 = t2137 * t33428;
    let t33431 = 0.17347256376410398924e1 * t33429 * t2140;
    let t33435 = 0.13170898365871023197e1 * t7976 * t1614;
    let t33437 = 0.13170898365871023197e1 * t29988 * t557;
    let t33441 = t33414 - t33416 - 0.17347256376410398924e1 * t8400 * t939 * t2122 * t8791 - t29973 - 0.52041769129231196772e1 * t7912 * t8411 - 0.52041769129231196772e1 * t2146 * t8004 * t8392 * t463 - 0.52041769129231196772e1 * t29977 + t33431 - 0.65854491829355115987e0 * t9010 * t1265 + t33435 - t33437 - 0.13877805101128319139e2 * t29982 + 0.8673628188205199462e0 * t29986 - 0.13170898365871023197e1 * t29989;
    (t33428, t33441)
}
