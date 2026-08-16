//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1830;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta420(t1025: f64, t10403: f64, t1041: f64, t10413: f64, t10909: f64, t10923: f64, t10927: f64, t14174: f64, t14180: f64, t14184: f64, t14189: f64, t14194: f64, t14198: f64, t14203: f64, t14207: f64, t14215: f64, t14222: f64, t14230: f64, t2960: f64, t3070: f64, t3117: f64, t4590: f64, t4609: f64, t973: f64, t14228: f64, t4337: f64, t10408: f64, t13510: f64, t13512: f64, t13514: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t13661: f64, t13665: f64, t13720: f64, t13722: f64, t13726: f64, t13729: f64, t13731: f64, t13734: f64, t13566: f64, t13602: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10636: f64, t13563: f64, t13569: f64, t13572: f64, t13575: f64, t13578: f64, t13581: f64, t13584: f64, t13587: f64, t13598: f64, t13613: f64) -> (f64, f64, f64, f64, f64) {
        let t14233 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1829(t1025, t10403, t1041, t10413, t10909, t10923, t10927, t14174, t14180, t14184, t14189, t14194, t14198, t14203, t14207, t14215, t14222, t14230, t2960, t3070, t3117, t4590, t4609, t973);
        let (t14234, t14235, t14238) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1830(t14228, t4337, t10408, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13720, t13722, t13726, t13729, t13731, t13734);
        let t14255 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1831(t13566, t13602, t10556, t10558, t10560, t10562, t10636, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
    (t14233, t14234, t14235, t14238, t14255)
}
