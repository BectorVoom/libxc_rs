//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 858/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk858<F: Float>(t1092: F, t9980: F, t9944: F, t9946: F, t9948: F, t9953: F, t9955: F, t9960: F, t9962: F, t9964: F, t9967: F, t9970: F, t9973: F, t9978: F) -> F {
    let t9981 = t1092 * t9980;
    let t9983 = F::new(0.22157690349980720138e-6) * t9944 + F::new(0.15176747947735985782e-6) * t9946 - F::new(0.26984257851074582721e-6) * t9948 - F::new(0.11795371371935910947e-5) * t9953 + F::new(0.61900849231692170544e-6) * t9955 - F::new(0.8894825648298215985e-9) * t9960 + F::new(0.18554144965277777779e-4) * t9962 + F::new(0.83413693878529023666e-4) * t9964 - F::new(0.37073828428874785365e-3) * t9967 + F::new(0.69504740211613770836e-4) * t9970 + F::new(0.69504740211613770836e-4) * t9973 + F::new(0.25745714186718600947e-6) * t9978 - F::new(0.14492726735651760868e-5) * t9981;
    t9983
}
