//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 558/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk558(t1062: f64, t3204: f64, t3116: f64, t4757: f64, t247: f64, t127: f64, t1663: f64, t371: f64, t1025: f64, t1063: f64, t1068: f64, t1675: f64, t3106: f64, t3112: f64, t3127: f64, t3174: f64, t3188: f64, t4818: f64, t4821: f64, t4825: f64, t4831: f64, t4834: f64) -> (f64, f64, f64, f64) {
    let t4837 = t3204 * t1062;
    let t4838 = t3116 * t4757;
    let t4839 = t247 * t4838;
    let t4845 = t371 * t127 * t1663;
    let t4846 = t1025 * t4845;
    let t4848 = 0.95275595817932748827e-4_f64 * t3112 + 0.14291339372689912324e-3_f64 * t3174 + 0.95275595817932748827e-4_f64 * t4818 + 0.14291339372689912324e-3_f64 * t4821 - 0.14291339372689912324e-3_f64 * t3127 * t4825 + 0.14291339372689912324e-3_f64 * t3188 * t1675 + 0.14291339372689912324e-3_f64 * t1063 * t4831 + 0.14291339372689912324e-3_f64 * t4834 * t1068 + 0.42874018118069736972e-3_f64 * t4837 * t4839 - 0.76220476654346199061e-3_f64 * t3106 * t1675 - 0.14291339372689912324e-3_f64 * t4846;
    (t4838, t4839, t4845, t4848)
}
