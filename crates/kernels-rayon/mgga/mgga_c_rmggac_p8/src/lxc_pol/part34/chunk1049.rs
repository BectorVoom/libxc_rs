//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1049/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1049(t78093: f64, t69437: f64, t69445: f64, t25820: f64, t77091: f64, t27048: f64, t77338: f64, t14434: f64, t1652: f64, t118: f64, t305: f64, t69417: f64, t69419: f64, t69424: f64, t76355: f64, t78083: f64, t78084: f64, t78087: f64, t78091: f64) -> (f64, f64) {
    let t78094 = 0.6818665413561335432e-1_f64 * t78093;
    let t78098 = 0.21819729323396273382e0_f64 * t69437;
    let t78099 = 0.54549323308490683456e-1_f64 * t69445;
    let t78100 = t25820 * t77091;
    let t78101 = 0.8980681276397856423e-1_f64 * t78100;
    let t78103 = 0.35922725105591425692e0_f64 * t27048 * t77338;
    let t78104 = t14434 * t1652;
    let t78107 = t78083 + 0.59871208509319042821e-1_f64 * t305 * t78084 + 0.59871208509319042821e-1_f64 * t305 * t78087 - t78091 + t78094 - 0.16566831523319392755e-1_f64 * t69417 + 0.49700494569958178265e-1_f64 * t69419 - 0.82834157616596963775e-1_f64 * t69424 + t78098 + t78099 + t78101 + t76355 + t78103 - 0.39914139006212695214e-1_f64 * t118 * t78104;
    (t78104, t78107)
}
