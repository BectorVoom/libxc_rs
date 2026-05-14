//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 780/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk780<F: Float>(t311: F, t9975: F, t474: F, t919: F, t3288: F, t7165: F, t1092: F, t9944: F, t9946: F, t9948: F, t9953: F, t9955: F, t9960: F, t9962: F, t9964: F, t9967: F, t9970: F, t9973: F) -> (F, F) {
    let t9976 = t311 * t9975;
    let t9977 = t474 * t919;
    let t9978 = t9976 * t9977;
    let t9980 = t3288 * t7165;
    let t9981 = t1092 * t9980;
    let t9983 = 0.22157690349980720138e-6 * t9944 + 0.15176747947735985782e-6 * t9946 - 0.26984257851074582721e-6 * t9948 - 0.11795371371935910947e-5 * t9953 + 0.61900849231692170544e-6 * t9955 - 0.8894825648298215985e-9 * t9960 + 0.18554144965277777779e-4 * t9962 + 0.83413693878529023666e-4 * t9964 - 0.37073828428874785365e-3 * t9967 + 0.69504740211613770836e-4 * t9970 + 0.69504740211613770836e-4 * t9973 + 0.25745714186718600947e-6 * t9978 - 0.14492726735651760868e-5 * t9981;
    (t9980, t9983)
}
