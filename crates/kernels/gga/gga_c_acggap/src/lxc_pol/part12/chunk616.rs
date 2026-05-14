//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 616/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk616<F: Float>(t4179: F, t4219: F, t4249: F, t5329: F, t449: F, t1215: F, t1659: F, t3865: F, t3880: F, t3884: F, t3886: F, t3890: F, t3893: F, t3897: F, t3900: F, t4130: F, t4133: F, t4139: F, t446: F, t557: F) -> (F, F, F) {
    let t5331 = t4179 + t4219 + t4249 + t5329;
    let t5332 = t449 * t5331;
    let t5336 = -t4130 - t4133 - 0.13170898365871023197e1 * t1215 * t1659 + 0.13170898365871023197e1 * t3880 - t4139 - 0.65854491829355115987e0 * t3865 * t557 + 0.65854491829355115987e0 * t3884 + 0.13170898365871023197e1 * t3886 + 0.13170898365871023197e1 * t3890 - 0.13170898365871023197e1 * t3893 - 0.65854491829355115987e0 * t446 * t5332 - 0.13170898365871023197e1 * t3897 - t3900;
    (t5331, t5332, t5336)
}
