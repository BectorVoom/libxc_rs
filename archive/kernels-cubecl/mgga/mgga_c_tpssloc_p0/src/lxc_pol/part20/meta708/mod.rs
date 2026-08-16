//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2710;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta708<F: Float>(t45616: F, t45648: F, t53774: F, t55315: F, t112: F, t16506: F, t1395: F, t2319: F, t111: F, t5363: F, t12521: F, t12524: F, t12529: F, t12532: F, t12813: F, t1401: F, t1458: F, t16521: F, t16524: F, t16535: F, t16538: F, t16541: F, t1851: F, t20173: F, t2363: F, t3938: F, t3941: F, t4072: F, t45557: F, t45560: F, t45782: F, t5371: F, t5376: F, t577: F, t671: F, t9416: F, t3946: F, t1858: F, t3931: F, t5381: F, t1404: F, t12513: F, t12537: F, t1396: F, t1398: F, t16507: F, t16546: F, t1852: F, t3: F, t39022: F, t39024: F, t39026: F, t39028: F, t3932: F, t45584: F, t45588: F, t5364: F, t580: F) -> F {
        let (t55317, t55364) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2710::<F>(t45616, t45648, t53774, t55315, t112, t16506, t1395, t2319, t111, t5363, t12521, t12524, t12529, t12532, t12813, t1401, t1458, t16521, t16524, t16535, t16538, t16541, t1851, t20173, t2363, t3938, t3941, t4072, t45557, t45560, t45782, t5371, t5376, t577, t671, t9416);
        let tv4rho41 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2711::<F>(t1851, t3946, t1858, t3931, t1395, t5381, t1404, t5363, t12513, t12537, t1396, t1398, t16507, t16546, t1852, t3, t39022, t39024, t39026, t39028, t3932, t45584, t45588, t5364, t55317, t55364, t580);
    tv4rho41
}
