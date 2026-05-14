//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 614/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk614<F: Float>(t1909: F, t310: F, t1220: F, t1658: F, t556: F, t1907: F, t315: F, t323: F, t1914: F, t3875: F, t463: F, t3880: F, t3890: F, t3893: F, t3900: F, t3902: F, t4103: F, t4130: F, t4133: F, t4139: F, t446: F, t557: F) -> (F, F, F) {
    let t6422 = t310 * t1909;
    let t6425 = t1220 * t556 * t1658;
    let t6434 = t315 * t1907;
    let t6435 = t6434 * t323;
    let t6438 = t3875 * t1914 * t463;
    let t6441 = 0.65854491829355115987e0 * t6422 - t4130 - t4133 + 0.26341796731742046394e1 * t446 * t6425 + 0.65854491829355115987e0 * t3880 - t4139 + 0.65854491829355115987e0 * t3890 - 0.13170898365871023197e1 * t4103 * t557 - 0.65854491829355115987e0 * t3893 - t3900 - 0.13170898365871023197e1 * t3902 - 0.65854491829355115987e0 * t6435 - 0.39512695097613069591e1 * t446 * t6438;
    (t6425, t6438, t6441)
}
