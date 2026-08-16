//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1154/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1154(t12739: f64, t12740: f64, t12741: f64, t12742: f64, t12745: f64, t12747: f64, t12748: f64, t12750: f64, t7954: f64, t7960: f64, t7972: f64, t7975: f64, t9886: f64, t9903: f64, t9906: f64, t9954: f64) -> f64 {
    let t12904 = t9886 - t12739 + t12740 + t12741 + t9903 - t9906 - t7954 - t12742 + t12745 - t7960 + t7972 + t7975 + t12747 - t12748 + t12750 - t9954;
    t12904
}
