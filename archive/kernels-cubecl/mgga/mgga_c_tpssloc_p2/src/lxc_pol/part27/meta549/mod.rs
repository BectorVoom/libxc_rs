//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1982;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1983;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1984;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta549<F: Float>(t26347: F, t3887: F, t22666: F, t7700: F, t1985: F, t1842: F, t3886: F, t1385: F, t22635: F, t1992: F, t6883: F, t7697: F, t1375: F, t16460: F, t2016: F, t26224: F, t26226: F, t26229: F, t26329: F, t26335: F, t26340: F, t26345: F, t3882: F, t5321: F, t568: F, t6963: F, t7729: F, t225: F, t7723: F, t2015: F, t5353: F, t22897: F, t5336: F, t22751: F, t7733: F, t1799: F, t22881: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26348, t26351, t26352, t26354, t26355, t26356, t26357, t26361) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1982::<F>(t26347, t3887, t22666, t7700, t1985, t1842, t3886, t1385, t22635, t1992, t6883, t7697);
        let t26364 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1983::<F>(t1375, t16460, t2016, t26224, t26226, t26229, t26329, t26335, t26340, t26345, t26348, t26352, t26357, t26361, t3882, t5321, t568, t6963, t7729);
        let (t26366, t26371, t26378, t26379, t26381, t26384) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1984::<F>(t225, t7723, t2015, t5353, t3887, t22897, t5336, t1992, t22751, t7733, t1799, t22881);
    (t26348, t26351, t26354, t26355, t26356, t26364, t26366, t26371, t26378, t26379, t26381, t26384)
}
