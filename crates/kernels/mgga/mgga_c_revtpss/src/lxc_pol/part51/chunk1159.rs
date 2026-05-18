//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1159/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1159<F: Float>(t120237: F, t31891: F, t1042: F, t120223: F, t120292: F, t120307: F, t120313: F, t120429: F, t120430: F, t120708: F, t120715: F, t1663: F, t1669: F, t19620: F, t2857: F, t3092: F, t3116: F, t31993: F, t32014: F, t33815: F, t385: F, t4181: F, t4782: F, t4786: F, t4787: F, t4803: F, t4808: F, t4911: F, t99638: F) -> F {
    let t126529 = t31891 * t120237;
    let t126546 = F::new(0.11156198762715303246e-2) * t120223 * t31993 * t3116 * t99638 - F::new(0.37645955677973955998e-3) * t32014 * t3092 * t385 * t2857 * t4181 - F::new(0.82638509353446690713e-4) * t120292 + F::new(0.18822977838986977999e-3) * t120429 * t120430 * t4782 + F::new(0.18822977838986977999e-3) * t120429 * t120430 * t4787 - F::new(0.56468933516960933998e-3) * t120313 * t120307 * t4911 + F::new(0.24791552806034007214e-3) * t126529 * t4803 - F::new(0.20659627338361672678e-3) * t126529 * t4808 - F::new(0.11156198762715303246e-2) * t120708 * t1042 * t33815 * t4786 + F::new(0.11156198762715303246e-2) * t120715 * t1042 * t1663 * t4786 + F::new(0.11156198762715303246e-2) * t120715 * t1042 * t1669 * t19620;
    t126546
}
