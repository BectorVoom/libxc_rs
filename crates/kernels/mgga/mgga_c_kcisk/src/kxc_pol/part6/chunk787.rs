//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 787/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk787<F: Float>(t28377: F, t706: F, t11495: F, t11630: F, t158: F, t173: F, t1809: F, t1850: F, t23225: F, t23229: F, t23231: F, t23234: F, t23236: F, t28312: F, t28368: F, t28621: F, t28624: F) -> (F,) {
    let t28627 = t706 * t28377;
    let t28642 = -0.93231700340333523768e-3 * t23225 - 0.2016525e-4 * t173 * t28621 + 0.21078e-1 * t158 * t28624 + 0.3513e-2 * t158 * t28627 - 0.5179538907796306876e-4 * t1850 * t28312 + 0.11955719325063177623e-1 * t1809 * t28312 - 0.62154466893555682512e-3 * t11630 * t28368 + 0.71734315950379065738e-1 * t11495 * t28368 + 0.26416666666666666666e-2 * t23229 - 0.352891875e-4 * t23231 + 0.4705225e-4 * t23234 + 0.70578375e-4 * t23236;
    (t28642,)
}
