//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 858/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk858(t1092: f64, t9980: f64, t9944: f64, t9946: f64, t9948: f64, t9953: f64, t9955: f64, t9960: f64, t9962: f64, t9964: f64, t9967: f64, t9970: f64, t9973: f64, t9978: f64) -> f64 {
    let t9981 = t1092 * t9980;
    let t9983 = 0.22157690349980720138e-6_f64 * t9944 + 0.15176747947735985782e-6_f64 * t9946 - 0.26984257851074582721e-6_f64 * t9948 - 0.11795371371935910947e-5_f64 * t9953 + 0.61900849231692170544e-6_f64 * t9955 - 0.8894825648298215985e-9_f64 * t9960 + 0.18554144965277777779e-4_f64 * t9962 + 0.83413693878529023666e-4_f64 * t9964 - 0.37073828428874785365e-3_f64 * t9967 + 0.69504740211613770836e-4_f64 * t9970 + 0.69504740211613770836e-4_f64 * t9973 + 0.25745714186718600947e-6_f64 * t9978 - 0.14492726735651760868e-5_f64 * t9981;
    t9983
}
