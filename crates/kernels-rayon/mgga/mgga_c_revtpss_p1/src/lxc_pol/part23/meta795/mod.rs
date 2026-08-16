//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta795 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2616;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta795(t10760: f64, t18409: f64, t9794: f64, t10777: f64, t10779: f64, t5984: f64, t837: f64, t18414: f64, t40799: f64, t18418: f64, t18392: f64, t236: f64, t807: f64, t854: f64, t18643: f64, t40731: f64, t10786: f64, t14931: f64, t61956: f64, t10811: f64, t18647: f64, t18511: f64, t40864: f64, t10905: f64, t18515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61981, t61985, t62012, t62015, t62021) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2616(t10760, t18409, t9794, t10777, t10779, t5984, t837, t18414, t40799, t18418, t18392, t236, t807, t854);
        let (t62029, t62033, t62045, t62056, t62058) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2617(t18643, t40731, t10779, t10786, t14931, t61956, t10811, t18647, t18511, t40864, t10905, t18515);
    (t61981, t61985, t62012, t62015, t62021, t62029, t62033, t62045, t62056, t62058)
}
