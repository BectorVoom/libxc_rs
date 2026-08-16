//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2132;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta570<F: Float>(t2986: F, t2990: F, t42771: F, t10346: F, t2987: F, t10190: F, t10245: F, t10250: F, t13779: F, t10255: F, t2989: F, t9258: F, t10337: F, t964: F, t340: F, t625: F, t221: F, t339: F, t344: F, t10195: F, t13784: F, t1887: F, t2262: F, t337: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42773, t42775, t42785, t42788, t42794, t42799) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2132::<F>(t2986, t2990, t42771, t10346, t2987, t10190, t10245, t10250, t13779, t10255, t2989, t9258);
        let (t42811, t42813, t42817, t42827, t42830) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2133::<F>(t10337, t964, t340, t625, t221, t339, t344, t10195, t13784, t2986, t1887, t2262, t337);
    (t42773, t42775, t42785, t42788, t42794, t42799, t42811, t42813, t42817, t42827, t42830)
}
