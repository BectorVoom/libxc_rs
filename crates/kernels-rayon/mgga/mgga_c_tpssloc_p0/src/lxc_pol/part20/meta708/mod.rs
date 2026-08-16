//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2710;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta708(t45616: f64, t45648: f64, t53774: f64, t55315: f64, t112: f64, t16506: f64, t1395: f64, t2319: f64, t111: f64, t5363: f64, t12521: f64, t12524: f64, t12529: f64, t12532: f64, t12813: f64, t1401: f64, t1458: f64, t16521: f64, t16524: f64, t16535: f64, t16538: f64, t16541: f64, t1851: f64, t20173: f64, t2363: f64, t3938: f64, t3941: f64, t4072: f64, t45557: f64, t45560: f64, t45782: f64, t5371: f64, t5376: f64, t577: f64, t671: f64, t9416: f64, t3946: f64, t1858: f64, t3931: f64, t5381: f64, t1404: f64, t12513: f64, t12537: f64, t1396: f64, t1398: f64, t16507: f64, t16546: f64, t1852: f64, t3: f64, t39022: f64, t39024: f64, t39026: f64, t39028: f64, t3932: f64, t45584: f64, t45588: f64, t5364: f64, t580: f64) -> f64 {
        let (t55317, t55364) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2710(t45616, t45648, t53774, t55315, t112, t16506, t1395, t2319, t111, t5363, t12521, t12524, t12529, t12532, t12813, t1401, t1458, t16521, t16524, t16535, t16538, t16541, t1851, t20173, t2363, t3938, t3941, t4072, t45557, t45560, t45782, t5371, t5376, t577, t671, t9416);
        let tv4rho41 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2711(t1851, t3946, t1858, t3931, t1395, t5381, t1404, t5363, t12513, t12537, t1396, t1398, t16507, t16546, t1852, t3, t39022, t39024, t39026, t39028, t3932, t45584, t45588, t5364, t55317, t55364, t580);
    tv4rho41
}
