//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 409/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk409(t6045: f64, t6832: f64, t1111: f64, t1412: f64, t1417: f64, t1701: f64, t238: f64, t3759: f64, t3766: f64, t3774: f64, t6034: f64, t6043: f64, t6053: f64, t6055: f64, t6759: f64, t6763: f64, t6767: f64, t6774: f64, t6778: f64, t6780: f64, t6784: f64, t6785: f64, t6795: f64, t6799: f64, t6805: f64, t6808: f64, t6809: f64, t6815: f64, t6821: f64, t6825: f64, t6829: f64) -> (f64, f64) {
    let t6833 = t6045 * t6832;
    let t6836 = -0.23254900946437792e-1_f64 * t3759 * t6759 + 2.0_f64 * t6763 + 0.11854761295685025975e-1_f64 * t1412 * t1111 - 2.0_f64 * t3766 * t6767 + 2.0_f64 * t6778 + 0.25845121844514357744e-4_f64 * t3774 * t6780 - 0.44455354858818847408e-2_f64 * t6784 * t1701 * t6785 - 0.52700762016626893448e-4_f64 * t238 * t6795 + 0.22227677429409423704e-2_f64 * t1417 * t6799 + 0.11854761295685025975e-1_f64 * t238 * t6774 + 0.22270151833971792333e-3_f64 * t6034 * t6805 - 0.38306165027777777778e-1_f64 * t6808 * t6045 * t6809 - 0.45411044255742331791e-3_f64 * t6815 * t6821 + 0.38306165027777777778e-1_f64 * t6043 * t6825 + 0.51074886703703703704e-1_f64 * t1417 * t6829 - t6053 - 0.6384360837962962963e-2_f64 * t6055 * t6833;
    (t6833, t6836)
}
