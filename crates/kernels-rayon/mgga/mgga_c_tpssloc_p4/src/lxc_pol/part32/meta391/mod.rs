//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1477;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta391(t16814: f64, t17048: f64, t858: f64, t225: f64, t5559: f64, t5657: f64, t865: f64, t2718: f64, t17022: f64, t218: f64, t5636: f64, t10110: f64, t1527: f64, t4300: f64, t259: f64, t2597: f64, t2713: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t5637: f64, t5658: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17049, t17050, t17052, t17056, t17057, t17060, t17063, t17064) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1477(t16814, t17048, t858, t225, t5559, t5657, t865, t2718, t17022, t218, t5636, t10110);
        let (t17069, t17070, t17079) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1478(t1527, t4300, t2718, t17050, t17052, t17057, t17060, t17064, t259, t2597, t2713, t4147, t4268, t4273, t4301, t5637, t5658, t855, t866);
    (t17049, t17050, t17052, t17056, t17057, t17060, t17063, t17064, t17069, t17070, t17079)
}
