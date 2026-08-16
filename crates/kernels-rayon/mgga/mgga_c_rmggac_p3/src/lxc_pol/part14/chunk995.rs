//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 995/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk995(t36254: f64, t40972: f64, t262: f64, t40805: f64, t7782: f64, t40808: f64, t35929: f64, t40738: f64, t4669: f64, t1587: f64, t664: f64, t25820: f64, t27094: f64, t305: f64, t321: f64, t35845: f64, t40960: f64, t40963: f64, t40967: f64, t40968: f64, t40970: f64, t5259: f64, t794: f64, t839: f64, t8946: f64) -> (f64, f64, f64, f64) {
    let t40973 = t36254 * t40972;
    let t40975 = t262 * t40805;
    let t40976 = t7782 * t40975;
    let t40978 = t262 * t40808;
    let t40979 = t35929 * t40978;
    let t40981 = t4669 * t40738;
    let t40983 = t664 * t1587;
    let t40988 = -0.11974241701863808564e1_f64 * t27094 * t8946 * t839 - 0.35922725105591425692e0_f64 * t25820 * t8946 * t794 + 0.59871208509319042821e-1_f64 * t305 * t40960 - 0.8980681276397856423e-1_f64 * t40963 + t40967 + 0.20455996240684006296e-1_f64 * t40968 - 0.54549323308490683457e-1_f64 * t40970 + 0.6818665413561335432e-1_f64 * t40973 + 0.72732431077987577943e-1_f64 * t40976 + 0.20455996240684006297e-1_f64 * t40979 + 0.8980681276397856423e-1_f64 * t40981 + 0.23948483403727617128e0_f64 * t5259 * t40983 * t321 + 0.15965655602485078085e0_f64 * t35845;
    (t40975, t40978, t40983, t40988)
}
