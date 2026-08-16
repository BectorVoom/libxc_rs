//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2056;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta588(t25273: f64, t540: f64, t1372: f64, t2019: f64, t9951: f64, t2018: f64, t9646: f64, t9723: f64, t26014: f64, t2689: f64, t807: f64, t9714: f64, t9703: f64, t3994: f64, t7028: f64, t9845: f64, t25240: f64, t3951: f64, t3964: f64, t25972: f64, t9761: f64, t2681: f64, t7269: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94519, t94520, t94523, t94526, t94527, t94530) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2056(t25273, t540, t1372, t2019, t9951, t2018, t9646, t9723, t26014, t2689, t807, t9714);
        let (t94534, t94537, t94540, t94542, t94545) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2057(t2018, t807, t9703, t3994, t7028, t9845, t25240, t3951, t3964, t25972, t9761, t2681, t7269, t820);
    (t94519, t94520, t94523, t94526, t94527, t94530, t94534, t94537, t94540, t94542, t94545)
}
