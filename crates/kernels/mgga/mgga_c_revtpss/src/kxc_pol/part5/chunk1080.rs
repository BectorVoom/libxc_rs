//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1080/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1080<F: Float>(t6041: F, t72: F, t686: F, t874: F, t10661: F, t10923: F, t10925: F, t10939: F, t10948: F, t10964: F, t10966: F, t10969: F, t10971: F, t14546: F, t14951: F, t14972: F, t1559: F, t18525: F, t18677: F, t18681: F, t18699: F, t4366: F, t4504: F, t6022: F, t820: F) -> (F,) {
    let t18761 = t6041 * t72;
    let t18763 = t874 * t18761 * t686;
    let t18782 = -0.26019841438354088051e-1 * t14951 - 0.73171657588172351096e-2 * t10923 + 0.65049603595885220126e-3 * t10925 + 0.26341796731742046394e1 * t4504 * t18681 * t4366 + 0.9757440539382783019e-2 * t18763 + 0.13170898365871023197e1 * t820 * t10661 * t6022 + t10939 + 0.13170898365871023197e1 * t4504 * t18699 * t4366 - t10948 - 0.13170898365871023197e1 * t820 * t14972 * t1559 - 0.65049603595885220126e-3 * t10964 + 0.73171657588172351096e-2 * t10966 + t10969 - t10971 - 0.39512695097613069591e1 * t14546 * t18677 * t18525 + 0.39512695097613069591e1 * t4504 * t18677 * t4366;
    (t18782,)
}
