//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 786/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk786<F: Float>(t11524: F, t11528: F, t11532: F, t16204: F, t16206: F, t16208: F, t16225: F, t16227: F, t165: F, t173: F, t28610: F, t28613: F, t28616: F, t1835: F, t28377: F, t28385: F) -> (F, F, F) {
    let t28619 = 0.10566666666666666666e-1 * t16204 + 0.117630625e-3 * t16206 - 0.32788e-1 * t16208 + 0.71734315950379065738e-1 * t16225 - 0.93231700340333523768e-3 * t16227 - t11524 + t11528 + t11532 - 0.30247875e-4 * t173 * t28610 - 0.4755e-2 * t165 * t28613 - 0.1585e-2 * t165 * t28616;
    let t28621 = t1835 * t28377;
    let t28624 = t1835 * t28385;
    (t28619, t28621, t28624)
}
