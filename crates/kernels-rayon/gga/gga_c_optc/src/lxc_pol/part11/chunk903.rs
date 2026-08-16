//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 903/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk903(t17004: f64, t894: f64, t16231: f64, t897: f64, t16988: f64, t297: f64, t935: f64, t313: f64, t16225: f64, t7870: f64, t14253: f64, t14280: f64, t14285: f64, t14290: f64, t14308: f64, t16962: f64, t16965: f64, t16969: f64, t16976: f64, t16981: f64, t16985: f64, t16991: f64, t16994: f64, t16998: f64, t17001: f64, t2640: f64, t2668: f64, t2678: f64, t7372: f64, t7386: f64, t862: f64, t893: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17005 = t894 * t17004;
    let t17008 = t897 * t16231;
    let t17009 = t894 * t17008;
    let t17013 = t16988 * t935 * t297;
    let t17014 = t313 * t17013;
    let t17017 = t7870 * t16225;
    let t17018 = t894 * t17017;
    let t17022 = 0.94667510637550784468e-1_f64 * t14253 + 0.27471835441379490507e2_f64 * t2668 * t16962 + 0.71000632978163088351e-1_f64 * t2640 * t16965 + 0.71000632978163088351e-1_f64 * t2640 * t16969 - 0.72443012414906315455e-2_f64 * t14280 + 0.36221506207453157727e-2_f64 * t14285 + 0.60369177012421929545e-2_f64 * t14290 + 0.11833438829693848058e0_f64 * t2640 * t16976 - 0.13735917720689745254e2_f64 * t2678 * t16981 - 0.1420012659563261767e0_f64 * t2640 * t16985 - 0.10629507243271336419e5_f64 * t7386 * t16991 - t862 * t16994 / 48.0_f64 + 0.10866451862235947318e-1_f64 * t893 * t16998 + t862 * t17001 / 72.0_f64 + 0.80492236016562572728e-2_f64 * t893 * t17005 + 0.18110753103726578864e-2_f64 * t893 * t17009 + 0.17715845405452227366e4_f64 * t7372 * t17014 - 0.18110753103726578864e-1_f64 * t893 * t17018 - 0.91572784804598301689e1_f64 * t14308;
    (t17005, t17008, t17009, t17013, t17017, t17018, t17022)
}
