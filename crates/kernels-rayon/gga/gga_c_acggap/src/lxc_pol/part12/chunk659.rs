//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 659/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk659(t4179: f64, t4219: f64, t4249: f64, t5329: f64, t449: f64, t1215: f64, t1659: f64, t3865: f64, t3880: f64, t3884: f64, t3886: f64, t3890: f64, t3893: f64, t3897: f64, t3900: f64, t4130: f64, t4133: f64, t4139: f64, t446: f64, t557: f64) -> (f64, f64, f64) {
    let t5331 = t4179 + t4219 + t4249 + t5329;
    let t5332 = t449 * t5331;
    let t5336 = -t4130 - t4133 - 0.13170898365871023197e1_f64 * t1215 * t1659 + 0.13170898365871023197e1_f64 * t3880 - t4139 - 0.65854491829355115987e0_f64 * t3865 * t557 + 0.65854491829355115987e0_f64 * t3884 + 0.13170898365871023197e1_f64 * t3886 + 0.13170898365871023197e1_f64 * t3890 - 0.13170898365871023197e1_f64 * t3893 - 0.65854491829355115987e0_f64 * t446 * t5332 - 0.13170898365871023197e1_f64 * t3897 - t3900;
    (t5331, t5332, t5336)
}
