//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2002;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta609(t23012: f64, t6573: f64, t1883: f64, t82045: f64, t6568: f64, t23205: f64, t82038: f64, t1914: f64, t40772: f64, t1054: f64, t2775: f64, t23326: f64, t6712: f64, t2770: f64, t1049: f64, t225: f64, t344: f64, t10189: f64, t1926: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82211, t82219, t82259, t82294, t82312, t82342, t82402) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2002(t23012, t6573, t1883, t82045, t6568, t23205, t82038, t1914, t40772, t1054, t2775, t23326, t6712);
        let (t82411, t82417, t82431) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2003(t1054, t2770, t1049, t225, t344, t10189, t1926, t221);
    (t82211, t82219, t82259, t82294, t82312, t82342, t82402, t82411, t82417, t82431)
}
