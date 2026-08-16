//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1159/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1159(t120237: f64, t31891: f64, t1042: f64, t120223: f64, t120292: f64, t120307: f64, t120313: f64, t120429: f64, t120430: f64, t120708: f64, t120715: f64, t1663: f64, t1669: f64, t19620: f64, t2857: f64, t3092: f64, t3116: f64, t31993: f64, t32014: f64, t33815: f64, t385: f64, t4181: f64, t4782: f64, t4786: f64, t4787: f64, t4803: f64, t4808: f64, t4911: f64, t99638: f64) -> f64 {
    let t126529 = t31891 * t120237;
    let t126546 = 0.11156198762715303246e-2_f64 * t120223 * t31993 * t3116 * t99638 - 0.37645955677973955998e-3_f64 * t32014 * t3092 * t385 * t2857 * t4181 - 0.82638509353446690713e-4_f64 * t120292 + 0.18822977838986977999e-3_f64 * t120429 * t120430 * t4782 + 0.18822977838986977999e-3_f64 * t120429 * t120430 * t4787 - 0.56468933516960933998e-3_f64 * t120313 * t120307 * t4911 + 0.24791552806034007214e-3_f64 * t126529 * t4803 - 0.20659627338361672678e-3_f64 * t126529 * t4808 - 0.11156198762715303246e-2_f64 * t120708 * t1042 * t33815 * t4786 + 0.11156198762715303246e-2_f64 * t120715 * t1042 * t1663 * t4786 + 0.11156198762715303246e-2_f64 * t120715 * t1042 * t1669 * t19620;
    t126546
}
